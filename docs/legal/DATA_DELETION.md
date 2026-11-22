# Ghal Bol data deletion

_Last updated: 20 November 2026_

This page is the **data deletion** URL for the Ghal Bol messenger (Google Play and Microsoft Store). Privacy: [ghalbol.com/privacy](https://ghalbol.com/privacy).

There is no cloud chat account to close. Transcripts live on each device. The delivery server keeps only temporary ciphertext until the recipient acknowledges it or it expires. Coordination presence expires when heartbeats stop.

## On the device

1. In the app, delete the local identity (Settings), **or**
2. Android: **Settings → Apps → Ghal Bol → Storage → Clear data**, **or** uninstall. Uninstall removes that install’s private storage.
3. Linux: uninstall, then delete `~/.local/share/com.ghalbol/` if it remains (debug builds: `com.ghalbol.debug`).
4. Windows: run Uninstall and choose to remove local data when the uninstaller asks. If a folder remains under your user profile for `com.ghalbol`, delete it.

Export a keystore backup first if you still need that identity.

## Servers we operate

- **Coordination** (`coord.ghalbol.com`): public key and endpoints, short TTL. No deletion request is required for presence; it expires.
- **Delivery** (`delivery.ghalbol.com`): opaque ciphertext until ack or expiry. We cannot decrypt it and we do not keep a transcript.

If you pointed the app at a coordination server you run yourself, that operator’s practices apply.

## Email we received

To delete a specific support thread, email [support@gotigin.com](mailto:support@gotigin.com) with the subject `Ghal Bol data deletion`. We will delete or anonymise that mailbox thread within **30 days**, except records we must keep for legal or abuse prevention.

## Contact

GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED  
[support@gotigin.com](mailto:support@gotigin.com)  
Gosiya Manzil, Darbargadh, Dhoraji, Rajkot, Gujarat 360410, India
