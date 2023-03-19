# Image Processing Service
This is a web service that processes images from external URLs in base64 encoded format, using the Rust programming language and the Rocket framework. The service also supports the use of the Protocol Buffers serialization format.

## Installation
1. Install Rust by following the instructions on the official website: https://www.rust-lang.org/tools/install
2. Clone this repository using git clone https://github.com/RandallYan/image-processing-service.git
3. In the project directory, run cargo build to build the project and its dependencies.
4. Run the service with cargo run.
## Usage
The service provides the following endpoints:

- `POST /process` - accepts a POST request with a Protocol Buffers payload containing the base64-encoded image to be processed. The payload should include the following fields:

  - `url` (string): the URL of the image to be processed.
  - `resize` (bool, optional): whether to resize the image. Defaults to false.
  - `width` (u32, optional): the desired width of the image after resizing. Defaults to 640.
  - `height` (u32, optional): the desired height of the image after resizing. Defaults to 480.
  - `rotate` (bool, optional): whether to rotate the image. Defaults to false.
  - `angle` (u32, optional): the angle (in degrees) by which to rotate the image. Defaults to 90.

  The response will be a Protocol Buffers payload containing the processed image in base64-encoded format.

- `GET /health` - returns a JSON object indicating the health status of the service. This endpoint can be used for monitoring and health checks.

## Examples
Here is an example of using curl to make a POST request to the /process endpoint:

```bash
$ curl -X POST -H "Content-Type: application/protobuf" --data-binary "@image.proto" "http://localhost:8000/process"
```
where image.proto is a file containing a Protocol Buffers payload like this:

```protobuf
syntax = "proto3";

message ImageRequest {
  string url = 1;
  bool resize = 2;
  uint32 width = 3;
  uint32 height = 4;
  bool rotate = 5;
  uint32 angle = 6;
}
```
The response will be a Protocol Buffers payload like this:

```protobuf
syntax = "proto3";

message ImageResponse {
  string data = 1;
}
```
## License
This project is licensed under the MIT License - see the LICENSE file for details.