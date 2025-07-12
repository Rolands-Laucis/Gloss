npm run tauri build
# only move the files, if the build didnt error
if ($?) {
    Write-Output "moving release folder..."
    Copy-Item src-tauri\target\release -Destination F:\Gloss -Recurse -Force
}

# Move-Item src-tauri\target\release F:\Muse -Force