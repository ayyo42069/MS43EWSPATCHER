# EWS IMMO Patcher for MS43

A simple desktop tool to remove the EWS (immobilizer) function from BMW MS43 ECU firmware files.

## Description

This tool allows users to load a binary firmware file from a BMW MS43 ECU, automatically detect the software version, and apply the necessary patches to disable the immobilizer. This is useful for engine swaps or in cases where the original EWS module has failed.

The application provides a clear user interface showing the patch status of the loaded file and allows for both applying and reverting patches. A hex viewer is included to show the exact byte-level changes for each patch.

**DISCLAIMER:** This tool modifies ECU firmware. Incorrect use or patching the wrong file can result in a non-functional ECU. Always back up your original firmware file before making any changes. The author is not responsible for any damage. Use at your own risk.

## Features

-   Automatic firmware version detection.
-   Support for common MS43 versions (ca430037, ca430056, ca430066, ca430069).
-   Clear three-state patch status display (Patched, Unpatched, Unknown).
-   One-click patch application and reversion.
-   Side-by-side hex viewer to inspect byte-level changes.
-   Detailed logging of all operations.

## Installation

Currently, the project must be built from source.

1.  **Install Rust:** If you don't have it, install the Rust toolchain from [rust-lang.org](https://www.rust-lang.org/).
2.  **Clone the repository:**
    ```sh
    git clone <repository-url>
    cd ews-immo-patcher
    ```
3.  **Build the project:**
    ```sh
    cargo build --release
    ```
4.  The executable will be located in the `target/release` directory.

## Usage

1.  Run the application executable.
2.  Click the "Browse..." button to load your MS43 firmware file (`.bin` or `.dat`).
3.  The tool will detect the version and display the status of the three required patches (Jump, Code, DTC).
    -   `✗` (Grey): The patch is not present (original state).
    -   `✓` (Green): The patch is present.
    -   `?` (Red): The file is in an unknown state and cannot be safely patched or reverted.
4.  Click on a patch status line to view the original and patched bytes in the hex viewer.
5.  If the status is fully unpatched, click "Apply Patches". You will be prompted to save the new patched file.
6.  If the status is fully patched, click "Revert". You will be prompted to save the reverted (original) file.

## Contributing

Contributions are welcome! Please feel free to fork the repository, make your changes, and submit a pull request.

1.  Fork the repository.
2.  Create your feature branch (`git checkout -b feature/AmazingFeature`).
3.  Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4.  Push to the branch (`git push origin feature/AmazingFeature`).
5.  Open a Pull Request.
