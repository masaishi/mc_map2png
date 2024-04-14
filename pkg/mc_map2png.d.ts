/* tslint:disable */
/* eslint-disable */
/**
* Processes an image from memory, extracts color data, and returns the resulting image bytes.
*
* # Arguments
*
* * `input_data` - A slice of bytes representing the input image data.
*
* # Returns
*
* A `Result` containing a `Vec<u8>` of the resulting image bytes if successful.
* If an error occurs, returns a `JsValue` with an error message.
* @param {Uint8Array} input_data
* @returns {Uint8Array}
*/
export function process_image_from_memory(input_data: Uint8Array): Uint8Array;
