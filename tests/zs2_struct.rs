//! Tests compression and decompression of structured data using OpenZL bindings
//!
//! Based on the C example zs2_struct.c from OpenZL which is Copyright (c) Meta Platforms, Inc. and affiliates.

use std::os::raw::c_void;

use openzl_sys::*;

// Helper function to check if a ZL_Report indicates an error
fn zl_is_error(report: ZL_Report) -> bool {
    // In OpenZL, errors are indicated when the _code field is non-zero
    // The union has _code field for errors and _value for success
    unsafe { report._code != ZL_ErrorCode_no_error }
}

// Helper function to extract the valid result from a ZL_Report
fn zl_valid_result(report: ZL_Report) -> usize {
    unsafe { report._value._value }
}

// Custom graph function that creates a compression graph specialized for structured data
// This is based on the sao_graph_v1 from the C example
unsafe extern "C" fn struct_compression_graph(compressor: *mut ZL_Compressor) -> ZL_GraphID {
    // Set format version to maximum
    let result = ZL_Compressor_setParameter(
        compressor,
        ZL_CParam_formatVersion,
        ZL_MAX_FORMAT_VERSION as i32,
    );
    if zl_is_error(result) {
        return ZL_GraphID {
            gid: ZL_StandardGraphID_illegal,
        };
    }

    // The SAO format consists of a 28-byte header followed by an array of structures
    let header_size: usize = 28;

    // Create compression graphs for different fields:

    // SRA0: Real*8 - B1950 Right Ascension (radians) - 8 bytes
    // Uses delta compression with little-endian interpretation
    let nodes = [
        ZL_NodeID {
            nid: ZL_StandardNodeID_convert_struct_to_num_le,
        },
        ZL_NodeID {
            nid: ZL_StandardNodeID_delta_int,
        },
    ];
    let sra0 = ZL_Compressor_registerStaticGraph_fromPipelineNodes1o(
        compressor,
        nodes.as_ptr(),
        nodes.len(),
        ZL_GraphID {
            gid: ZL_StandardGraphID_field_lz,
        },
    );

    // SDEC0: Real*8 - B1950 Declination (radians) - 8 bytes
    // Uses transpose split
    let sdec0 = ZL_Compressor_registerStaticGraph_fromNode1o(
        compressor,
        ZL_NodeID {
            nid: ZL_StandardNodeID_transpose_split,
        },
        ZL_GraphID {
            gid: ZL_StandardGraphID_zstd,
        },
    );

    // Token compression for struct types
    let token_compress = ZL_Compressor_registerTokenizeGraph(
        compressor,
        ZL_Type_struct,
        false,
        ZL_GraphID {
            gid: ZL_StandardGraphID_field_lz,
        },
        ZL_GraphID {
            gid: ZL_StandardGraphID_field_lz,
        },
    );

    // Numeric huffman compression
    let huffman_inner = ZL_Compressor_registerTokenizeGraph(
        compressor,
        ZL_Type_numeric,
        false,
        ZL_GraphID {
            gid: ZL_StandardGraphID_huffman,
        },
        ZL_GraphID {
            gid: ZL_StandardGraphID_huffman,
        },
    );
    let num_huffman = ZL_Compressor_registerStaticGraph_fromNode1o(
        compressor,
        ZL_NodeID {
            nid: ZL_StandardNodeID_convert_struct_to_num_le,
        },
        huffman_inner,
    );

    // IS: Character*2 - Spectral type - 2 bytes
    let is = num_huffman;
    // MAG: Integer*2 - V Magnitude * 100 - 2 bytes
    let mag = num_huffman;
    // XRPM: Real*4 - R.A. proper motion - 4 bytes
    let xrpm = token_compress;
    // XDPM: Real*4 - Dec. proper motion - 4 bytes
    let xdpm = token_compress;

    // Split the structure by fields: 8 + 8 + 2 + 2 + 4 + 4 = 28 bytes
    let field_sizes = [8usize, 8, 2, 2, 4, 4];
    let field_graphs = [sra0, sdec0, is, mag, xrpm, xdpm];
    let split_structure = ZL_Compressor_registerSplitByStructGraph(
        compressor,
        field_sizes.as_ptr(),
        field_graphs.as_ptr(),
        6,
    );

    // Split the entire data: header (28 bytes stored as-is) + structures (compressed)
    let sizes = [header_size, 0]; // 0 means "rest of data"
    let graphs = [
        ZL_GraphID {
            gid: ZL_StandardGraphID_store,
        },
        split_structure,
    ];
    ZL_Compressor_registerSplitGraph(
        compressor,
        ZL_Type_serial,
        sizes.as_ptr(),
        graphs.as_ptr(),
        2,
    )
}

// Graph function with compression parameters
unsafe extern "C" fn graph_with_parameters(compressor: *mut ZL_Compressor) -> ZL_GraphID {
    let graph_id = struct_compression_graph(compressor);
    let illegal = ZL_GraphID {
        gid: ZL_StandardGraphID_illegal,
    };
    if graph_id.gid == illegal.gid {
        return illegal;
    }

    // Set compression level
    let result = ZL_Compressor_setParameter(compressor, ZL_CParam_compressionLevel, 3);
    if zl_is_error(result) {
        return illegal;
    }

    graph_id
}

// Compress data using the custom graph function
fn compress(src: &[u8]) -> Result<Vec<u8>, String> {
    unsafe {
        let src_size = src.len();

        // Calculate compress bound
        let dst_capacity = ZL_compressBound(src_size);
        let mut dst = vec![0u8; dst_capacity];

        let report = ZL_compress_usingGraphFn(
            dst.as_mut_ptr() as *mut c_void,
            dst_capacity,
            src.as_ptr() as *const c_void,
            src_size,
            Some(graph_with_parameters),
        );

        if zl_is_error(report) {
            return Err(format!(
                "Compression failed with error code: {:?}",
                report._code
            ));
        }

        let compressed_size = zl_valid_result(report);
        dst.truncate(compressed_size);
        Ok(dst)
    }
}

// Decompress data
fn decompress(compressed: &[u8]) -> Result<Vec<u8>, String> {
    unsafe {
        // Get the decompressed size
        let size_report =
            ZL_getDecompressedSize(compressed.as_ptr() as *const c_void, compressed.len());

        if zl_is_error(size_report) {
            return Err(format!(
                "Failed to get decompressed size, error code: {:?}",
                size_report._code
            ));
        }

        let decompressed_size = zl_valid_result(size_report);
        let mut dst = vec![0u8; decompressed_size];

        let report = ZL_decompress(
            dst.as_mut_ptr() as *mut c_void,
            decompressed_size,
            compressed.as_ptr() as *const c_void,
            compressed.len(),
        );

        if zl_is_error(report) {
            return Err(format!(
                "Decompression failed with error code: {:?}",
                report._code
            ));
        }

        let actual_size = zl_valid_result(report);
        assert_eq!(actual_size, decompressed_size);

        Ok(dst)
    }
}

#[test]
fn test_compress_decompress_roundtrip() {
    // Create test data that simulates an array of structures
    // Each "structure" is 28 bytes (similar to the SAO format)
    const STRUCT_SIZE: usize = 28;
    const NUM_STRUCTS: usize = 100;
    const HEADER_SIZE: usize = 28;

    let mut input = Vec::new();

    // Add a 28-byte header
    input.extend_from_slice(&[0u8; HEADER_SIZE]);

    // Add structured data
    for i in 0..NUM_STRUCTS {
        let mut structure = [0u8; STRUCT_SIZE];
        // Fill with some pattern data
        structure[0..8].copy_from_slice(&(i as u64).to_le_bytes());
        structure[8..16].copy_from_slice(&((i * 2) as u64).to_le_bytes());
        structure[16] = (i % 256) as u8;
        structure[17] = ((i + 1) % 256) as u8;
        structure[18..22].copy_from_slice(&((i as f32) * 0.5).to_le_bytes());
        structure[22..26].copy_from_slice(&((i as f32) * 0.25).to_le_bytes());
        structure[26] = ((i * 3) % 256) as u8;
        structure[27] = ((i * 5) % 256) as u8;

        input.extend_from_slice(&structure);
    }

    let input_size = input.len();
    println!("Input size: {} bytes", input_size);
    println!("Compress bound: {} bytes", unsafe {
        ZL_compressBound(input_size)
    });

    // Compress the data
    let compressed = compress(&input).expect("Compression should succeed");
    let compressed_size = compressed.len();
    println!("Compressed size: {} bytes", compressed_size);

    // Verify compression actually reduced size
    assert!(
        compressed_size < input_size,
        "Compression should reduce size"
    );

    // Decompress the data
    let decompressed = decompress(&compressed).expect("Decompression should succeed");

    // Verify round-trip integrity
    assert_eq!(
        decompressed.len(),
        input_size,
        "Decompressed size should match original"
    );
    assert_eq!(
        decompressed, input,
        "Decompressed data should match original"
    );

    println!("Round-trip test passed!");
    println!(
        "Compression ratio: {:.2}%",
        (compressed_size as f64 / input_size as f64) * 100.0
    );
}
