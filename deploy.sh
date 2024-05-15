#!/bin/bash

# Function to display error message and exit
error_exit() {
    echo "$1" 1>&2
    exit 1
}

# Ask for the project name
project_name="rusty_bunker"

# Run tailwindcss
tailwindcss -i styles/input.css -o styles/output.css --minify || error_exit "tailwindcss failed."

# Run trunk build --release
trunk build --release || error_exit "trunk build --release failed."

# Check if 'dist' folder exists
if [ ! -d "dist" ]; then
    error_exit "'dist' folder not found!"
fi


# Create a folder with the project name and copy 'dist' folder into it
mkdir "$project_name" || error_exit "Failed to create folder with project name."
# Copy the contents of 'dist' folder into the folder with the project name
cp -r dist/* "$project_name" || error_exit "Failed to copy 'dist' folder contents."

# Hardcoded username and server
username="illuminodes"
hostname="50.116.20.217"

# SCP the folder to the server
scp -r "$project_name" "$username@$hostname:~/" || error_exit "SCP failed."

# Clean up
rm -r "$project_name"
rm -r dist

echo "Deployment successful and folders cleaned up."

