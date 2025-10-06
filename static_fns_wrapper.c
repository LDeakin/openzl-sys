#include "openzl/include/openzl/openzl.h"

// Static wrappers

ZL_GraphID ZL_Result_ZL_GraphID_extract__extern(const ZL_Result_ZL_GraphID result, const ZL_Error *const error) { return ZL_Result_ZL_GraphID_extract(result, error); }
ZL_NodeID ZL_Result_ZL_NodeID_extract__extern(const ZL_Result_ZL_NodeID result, const ZL_Error *const error) { return ZL_Result_ZL_NodeID_extract(result, error); }
size_t ZL_Result_size_t_extract__extern(const ZL_Result_size_t result, const ZL_Error *const error) { return ZL_Result_size_t_extract(result, error); }
int ZL_isError__extern(ZL_Report report) { return ZL_isError(report); }
size_t ZL_validResult__extern(ZL_Report report) { return ZL_validResult(report); }
ZL_ErrorCode ZL_errorCode__extern(ZL_Report report) { return ZL_errorCode(report); }
ZL_Report ZL_returnValue__extern(size_t s) { return ZL_returnValue(s); }
ZL_Report ZL_returnSuccess__extern(void) { return ZL_returnSuccess(); }
size_t ZL_compressBound__extern(size_t totalSrcSize) { return ZL_compressBound(totalSrcSize); }
const ZL_Data * ZL_codemodInputAsData__extern(const ZL_Input *input) { return ZL_codemodInputAsData(input); }
const ZL_Input * ZL_codemodDataAsInput__extern(const ZL_Data *data) { return ZL_codemodDataAsInput(data); }
ZL_Data * ZL_codemodMutInputAsData__extern(ZL_Input *input) { return ZL_codemodMutInputAsData(input); }
ZL_Input * ZL_codemodMutDataAsInput__extern(ZL_Data *data) { return ZL_codemodMutDataAsInput(data); }
const ZL_Input ** ZL_codemodDatasAsInputs__extern(const ZL_Data **datas) { return ZL_codemodDatasAsInputs(datas); }
const ZL_Data ** ZL_codemodInputsAsDatas__extern(const ZL_Input **inputs) { return ZL_codemodInputsAsDatas(inputs); }
ZL_DataID ZL_Input_id__extern(const ZL_Input *input) { return ZL_Input_id(input); }
ZL_Type ZL_Input_type__extern(const ZL_Input *input) { return ZL_Input_type(input); }
size_t ZL_Input_numElts__extern(const ZL_Input *input) { return ZL_Input_numElts(input); }
size_t ZL_Input_eltWidth__extern(const ZL_Input *input) { return ZL_Input_eltWidth(input); }
size_t ZL_Input_contentSize__extern(const ZL_Input *input) { return ZL_Input_contentSize(input); }
const void * ZL_Input_ptr__extern(const ZL_Input *input) { return ZL_Input_ptr(input); }
const uint32_t * ZL_Input_stringLens__extern(const ZL_Input *input) { return ZL_Input_stringLens(input); }
ZL_IntMetadata ZL_Input_getIntMetadata__extern(const ZL_Input *input, int key) { return ZL_Input_getIntMetadata(input, key); }
ZL_Report ZL_Input_setIntMetadata__extern(ZL_Input *input, int key, int value) { return ZL_Input_setIntMetadata(input, key, value); }
ZL_GraphPerformance ZL_Result_ZL_GraphPerformance_extract__extern(const ZL_Result_ZL_GraphPerformance result, const ZL_Error *const error) { return ZL_Result_ZL_GraphPerformance_extract(result, error); }
ZL_EdgeList ZL_Result_ZL_EdgeList_extract__extern(const ZL_Result_ZL_EdgeList result, const ZL_Error *const error) { return ZL_Result_ZL_EdgeList_extract(result, error); }
ZL_NodeID ZL_Node_interpretAsLE__extern(size_t bitWidth) { return ZL_Node_interpretAsLE(bitWidth); }
ZL_CompressorDeserializer_Dependencies ZL_Result_ZL_CompressorDeserializer_Dependencies_extract__extern(const ZL_Result_ZL_CompressorDeserializer_Dependencies result, const ZL_Error *const error) { return ZL_Result_ZL_CompressorDeserializer_Dependencies_extract(result, error); }
ZL_Data * ZL_codemodOutputAsData__extern(ZL_Output *output) { return ZL_codemodOutputAsData(output); }
ZL_Output * ZL_codemodDataAsOutput__extern(ZL_Data *data) { return ZL_codemodDataAsOutput(data); }
const ZL_Data * ZL_codemodConstOutputAsData__extern(const ZL_Output *output) { return ZL_codemodConstOutputAsData(output); }
const ZL_Output * ZL_codemodConstDataAsOutput__extern(const ZL_Data *data) { return ZL_codemodConstDataAsOutput(data); }
const ZL_Output ** ZL_codemodConstDatasAsOutputs__extern(const ZL_Data **datas) { return ZL_codemodConstDatasAsOutputs(datas); }
ZL_Data ** ZL_codemodOutputsAsDatas__extern(ZL_Output **outputs) { return ZL_codemodOutputsAsDatas(outputs); }
ZL_Type ZL_Output_type__extern(const ZL_Output *output) { return ZL_Output_type(output); }
ZL_DataID ZL_Output_id__extern(const ZL_Output *output) { return ZL_Output_id(output); }
void * ZL_Output_ptr__extern(ZL_Output *output) { return ZL_Output_ptr(output); }
const void * ZL_Output_constPtr__extern(const ZL_Output *output) { return ZL_Output_constPtr(output); }
uint32_t * ZL_Output_stringLens__extern(ZL_Output *output) { return ZL_Output_stringLens(output); }
const uint32_t * ZL_Output_constStringLens__extern(const ZL_Output *output) { return ZL_Output_constStringLens(output); }
uint32_t * ZL_Output_reserveStringLens__extern(ZL_Output *output, size_t numStrings) { return ZL_Output_reserveStringLens(output, numStrings); }
ZL_Report ZL_Output_commit__extern(ZL_Output *output, size_t numElts) { return ZL_Output_commit(output, numElts); }
ZL_Report ZL_Output_setIntMetadata__extern(ZL_Output *output, int key, int value) { return ZL_Output_setIntMetadata(output, key, value); }
ZL_IntMetadata ZL_Output_getIntMetadata__extern(const ZL_Output *output, int key) { return ZL_Output_getIntMetadata(output, key); }
