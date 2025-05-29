#!/bin/bash

echo "Running"


cd target || {
    echo "Error: Failed to change to 'target' directory"
    exit 1
}


cd debug || {
    echo "Error: Failed to change to 'debug' directory"
    exit 1
}


ls


if [[ -f SXsv ]]; then
    # Move SXsv to /usr/local/bin with sudo
    sudo mv SXsv /usr/local/bin || {
        echo "Error: Failed to move SXsv to /usr/local/bin"
        exit 1
    }
else
    echo "Error: SXsv file not found in $(pwd)"
    exit 1
fi

echo "Exit"