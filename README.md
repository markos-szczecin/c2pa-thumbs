# c2pa-thumbs
Fetch all thumbs from file with c2pa

## Installation

cargo install --git https://github.com/SmartFrame-Technologies/c2pa-thumbs.git --tag 0.1.0

## How to use it

Run from command line
```shell
c2pa-thumbs {file-path}
```

## Result

Output of command containt json encoded with 2 keys:
```json
{
    "thumnail": "main_thumnail_base64_encoded_source",
    "thumnails": [
        "ingrendient_thumbnail_base64_encoded_source",
        "ingrendient_1_thumbnail_base64_encoded_source",
        ...
    ]
}
```