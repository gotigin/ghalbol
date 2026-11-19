ghalbol-linux-x64.tar.gz is written here by ./scripts/package_linux_release.sh
(production env, release build) and served by Firebase at
/downloads/ghalbol-linux-x64.tar.gz. It is gitignored — build it, then
./sites/ghalbol/scripts/deploy-firebase.sh (or preview locally first).

The archive holds the binary, its ghalbol_app/ and makepad_widgets/ resource
folders, install.sh / uninstall.sh (per-user, no root) and run.sh (try in place).
