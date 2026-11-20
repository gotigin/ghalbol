# Ghal Bol — product story

**Gotigin** is the trademark. **GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED** is the company. **Ghal Bol** is the communication brand. The product that ships is the **Ghal Bol messenger** (public name Ghal Bol, `com.ghalbol`).

The name means communication. The product is for two people who want to talk without handing a phone number to a cloud archive.

## What a person can do today

- Create a device identity (no phone number, no social login)
- Invite someone with a QR or `https://ghalbol.com/connect/…`
- Send 1:1 text. The delivery mailbox holds only ciphertext until the other device acknowledges it
- See delivery and read receipts when the other device actually sends them
- Place a voice or video call. The call prefers a direct path and can use the coordination bridge when NAT requires it. The bridge does not decrypt media
- Keep the transcript on this device. The other person’s device has its own copy

Guest scans host. Mutual QR is not required.

## What this is not

- Not a phone-number messenger and not an SMS app
- Not a group chat, feed, or public profile
- Not a cloud backup of your chats. Uninstall removes the local copy. We cannot restore a forgotten app password
- Not spelled **GhalBol**. That camel case is not a brand

## Where it runs

| Surface | Status |
| --- | --- |
| Android (`com.ghalbol`) | Play package path: [`PUBLISH_PLAY_STORE.md`](PUBLISH_PLAY_STORE.md). Version code **10** / **1.1.0** is the next upload after the earlier Flutter releases (codes through 9) |
| Linux x86_64 | Public tarball on ghalbol.com when packaged |
| Windows | Per-user folder install (`Setup.bat`). Microsoft Store MSIX is not packed in this repo yet — [`PUBLISH_WINDOWS_STORE.md`](PUBLISH_WINDOWS_STORE.md) |
| ghalbol.com | Brand site, invite handoff, privacy and store URLs |

Design detail lives in [`DESIGN.md`](DESIGN.md). Names live in [`GHALBOL_BRAND.md`](GHALBOL_BRAND.md).
