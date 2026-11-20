# Publish Ghal Bol on the Microsoft Store

Desktop id: **`com.ghalbol`**  
Product name: **Ghal Bol**  
Publisher shown in the sideload uninstall entry: **GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED**

## Store URLs

| Partner Center field | URL |
| --- | --- |
| Privacy policy | https://ghalbol.com/privacy |
| Website | https://ghalbol.com |
| Support | https://ghalbol.com/support · mearaj@gotigin.com |
| Terms | https://ghalbol.com/terms |

## What this repo packs today

```bash
./scripts/package-windows.sh
```

Output: `dist/windows/` — per-user install via **Setup.bat** (no admin). It registers Start Menu shortcuts, an Apps & features entry, sign-in autostart, and the `ghalbol://` invite handler. Data stays under the user profile for `com.ghalbol`. This package is for sideload and USB. It is **not** an MSIX.

`apps/ghalbol/scripts/windows-dev.sh` is the debug-id package (`com.ghalbol.debug`).

## Microsoft Store (MSIX) — not packed yet

There is no `msix.sh` in this workspace. Partner Center re-signs an MSIX; a folder + Setup.bat cannot be the Store package.

When the app name is reserved:

1. Copy **Package/Identity/Publisher** (the `CN=…` value) from Partner Center Product identity. Do not invent it.
2. Pick Identity **Name** once and never rename it after the first publish. If this is the same Partner Center account as the other Gotigin desktop apps, the matching shape is `MearajBhagad.GhalBol`. Confirm that string in Partner Center before the first upload.
3. Add an MSIX packer only after that identity is copied. Publisher display name for the Store listing should be the company, **GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED**, unless Partner Center already shows a different display name that must match the certificate.
4. Age rating: not directed at children. Capabilities: network (`runFullTrust` if the package is a classic Win32 app, same pattern as the other desktop apps).
5. Privacy URL https://ghalbol.com/privacy before certification.

Until that packer exists, ship Windows as the Setup.bat folder and keep the Store listing unpublished.

## Related

- Play: [`PUBLISH_PLAY_STORE.md`](PUBLISH_PLAY_STORE.md)
- Legal: [`legal/README.md`](legal/README.md)
- Story: [`STORY.md`](STORY.md)
