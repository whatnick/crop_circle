# Crop-Circle - Not the alien made kind

- Pick up an image via a public URL / S3 bucket key
- Handle images in both `.jpg` and `.png` format
- Crop image to the largest circle detected via [OpenCV Hough Transform](https://docs.opencv.org/2.4/doc/tutorials/imgproc/imgtrans/hough_circle/hough_circle.html)
- In true Crop-Circle fashion replace any green pixels with transparency
- Save image back to S3 bucket specified in config with `_crop.png` suffix
- Run entire pipeline in serverless mode in Azure / AWS
- Able to run in CLI mode with string argument for S3 bucket key
