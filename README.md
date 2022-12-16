# c2pa-thumbs
Fetch all thumbnails from the file signed with C2PA.

## Installation

cargo install --git https://github.com/SmartFrame-Technologies/c2pa-thumbs.git

## How to use it

Run from the command line
```shell
c2pa-thumbs {file-path}
```

## Result

The output of the command contains JSON encoded with 2 keys:
```json
{
    "thumbnail": "main_thumbnail_base64_encoded_source",
    "thumbnails": [
        "ingrendient_thumbnail_base64_encoded_source",
        "ingrendient_1_thumbnail_base64_encoded_source",
        ...
    ]
}
```

## Dependencies

This application uses the [c2pa](https://github.com/contentauth/c2pa-rs) repository. 
Look into the Cargo.toml file to see the full list of dependencies.
