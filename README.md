# Awan
Awan is a simple web apps for manage Amazon S3 objects. That provide services can be used to store and retrieve data, at any time, from anywhere on the web.

## Ready APIs
| API | Description | Docs/Reference
| --- | ----------- | :-------------: |
| `/s3/objects` | Get list the contents of an S3 bucket | [Docs](https://docs.aws.amazon.com/cli/latest/reference/s3api/list-objects.html)
| `/s3/presigned?filekey=<filekey>` | Generating a presigned URL to download/sharing a file | [Docs](https://docs.aws.amazon.com/AmazonS3/latest/userguide/ShareObjectPreSignedURL.html)
| `/s3/delete?filekey=<filekey>` | Delete file/object from an S3 path. | [Docs](https://docs.aws.amazon.com/cli/latest/reference/s3api/delete-object.html)
| `/s3/bucket` | Get a name and region of a bucket | [Docs](https://docs.aws.amazon.com/AWSJavaScriptSDK/latest/AWS/S3.html#getBucketWebsite-property)

## Currently working on 
[ ] Render list objects in table. 

## Commands