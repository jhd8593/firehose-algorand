# Demo Recording Upload Instructions

## Upload the Asciinema Recording

To make the demo recording accessible in the README, upload it to asciinema.org:

1. **Upload the recording:**
   ```bash
   asciinema upload algorand-substreams-demo.cast
   ```

2. **Get the recording ID** from the upload response (e.g., `abc123def`)

3. **Update the README** with the correct recording ID:
   ```markdown
   [![asciicast](https://asciinema.org/a/YOUR_RECORDING_ID.png)](https://asciinema.org/a/YOUR_RECORDING_ID)
   ```

## Current Recording File

- **File**: `algorand-substreams-demo.cast`
- **Size**: 375 bytes
- **Content**: Live demonstration of Algorand Firehose Substreams with JWT authentication

## What the Demo Shows

The recording demonstrates:
- ✅ JWT token authentication working
- ✅ Substreams connecting to The Graph's endpoint
- ✅ Real-time processing of Algorand blocks (50500000-50500003)
- ✅ JSON output streaming to console
- ✅ TraceID and processing status

This provides visual proof that the Algorand Firehose Substreams integration is fully functional.
