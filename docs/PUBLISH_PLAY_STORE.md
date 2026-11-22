# Publish Ghal Bol on Google Play

Application id: **`com.ghalbol`**  
Public name: **Ghal Bol**  
Developer: **GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED**  
Listing copy: [`PLAY_STORE_LISTING.md`](PLAY_STORE_LISTING.md)  
Data safety: [`DATA_SAFETY.md`](DATA_SAFETY.md)

Play already accepted the earlier Flutter app through **version code 9** (1.0.1). The Makepad app in this repo is **version code 10 / version name 1.1.0** in `apps/ghalbol/Cargo.toml`. Each later upload increments `version_code` by 1. Never use a timestamp code. Never decrease it. Sign with the **same Play upload key** as those Flutter releases.

## Store URLs

| Console field | URL |
| --- | --- |
| Privacy policy | https://ghalbol.com/privacy |
| Terms | https://ghalbol.com/terms |
| Support URL / email | https://ghalbol.com/support · support@gotigin.com |
| Account / data deletion | https://ghalbol.com/data-deletion |
| Website | https://ghalbol.com |

Deploy before review: `./sites/ghalbol/scripts/deploy-firebase.sh`.

## Keystore

| Kind | Default |
| --- | --- |
| Upload key | `$HOME/ghalbol-release.jks` |
| Password | `$HOME/ghalbol-release.jks.password` (one line, mode 600) |
| Alias | `$HOME/ghalbol-release.jks.makepad` if present, else `upload` |

Overrides: `GHALBOL_ANDROID_KEYSTORE`, `GHALBOL_ANDROID_KEYSTORE_PASS`, `GHALBOL_ANDROID_KEY_ALIAS`. The password is passed as `MAKEPAD_KEYSTORE_PASS` and is not printed.

If the keystore is missing, `./scripts/build_android_app.sh --aab` still builds and warns: that AAB is debug-signed and **Play rejects it**.

Back up the `.jks` outside the repo. A lost upload key blocks updates to `com.ghalbol`.

App Links fingerprint: `./scripts/print_android_assetlinks_sha.sh "$HOME/ghalbol-release.jks" upload`  
Play App Signing uses the **app signing** certificate in `sites/ghalbol/static/.well-known/assetlinks.json`, which can differ from the upload key. See [`ANDROID_APP_LINKS.md`](ANDROID_APP_LINKS.md).

## Build

```bash
./scripts/build_android_app.sh --aab
```

`--release` / `--apk` are device or sideload builds (production env). `--aab` is the Play bundle. Artifacts land under `target/makepad-android-apk/`.

`minSdkVersion` is **26** (Cargo). `targetSdkVersion` is **35**, set by the pinned cargo-makepad (`rev 1f60033`), not by Cargo.toml. Confirm Play’s current target-API rule at upload time. Raising it is a toolchain bump.

Manifest already sets `android:allowBackup="false"`, camera/mic as optional features, `ghalbol://connect`, and verified `https://ghalbol.com/connect`.

## Console checklist

1. Listing from [`PLAY_STORE_LISTING.md`](PLAY_STORE_LISTING.md)
2. Icon and screenshots from the running app
3. Privacy URL https://ghalbol.com/privacy
4. Data safety from [`DATA_SAFETY.md`](DATA_SAFETY.md)
5. Content rating — 1:1 chat, not a public feed; not directed at children
6. Upload the release-signed AAB at version code **greater than** the highest code already on Play (10 if 9 is still the latest)
7. Roll out internal testing before production

## Related

- Windows: [`PUBLISH_WINDOWS_STORE.md`](PUBLISH_WINDOWS_STORE.md)
- Legal index: [`legal/README.md`](legal/README.md)
- Release notes draft: [`PLAY_STORE_LISTING.md`](PLAY_STORE_LISTING.md)
