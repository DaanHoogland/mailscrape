# mailscrape
Get statistics for an Apache mailing list from a pony list site

## Description
This tool fetches and analyzes statistics from Apache mailing lists, providing detailed information about email activity, participants, and threads over a specified time period.

## Installation
Ensure you have Rust installed on your system, then clone this repository and build using cargo:

## Usage
Run the tool from the command line with the required arguments:

### Arguments
- `--start-date`, `-s`: Start date in YYYY-MM-DD format (required)
- `--end-date`, `-e`: End date in YYYY-MM-DD format (required)
- `--list`, `-l`: Name of the mailing list (default: "dev")
- `--domain`, `-d`: Domain of the mailing list (default: "cloudstack.apache.org")

### Example


## Output
The tool provides a detailed analysis including:
- Daily activity breakdown showing emails, participants, and threads
- Total counts for emails, participants, and threads
- Average daily statistics
- Period summary and list information

Example output:

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
Apache License 2.0

For more details, see the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) text.
