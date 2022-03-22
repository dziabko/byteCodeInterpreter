find_files() {
        find . -type f -name "*.$1" -exec wc -l {} \;
}

# ran using 'find_files {FILE_EXTENSION}'
# ex: 'find_files rs'