# Play Data safety — Ghal Bol

Paste guidance for Play Console. The public policy is https://ghalbol.com/privacy ([`PRIVACY_POLICY.md`](PRIVACY_POLICY.md)).

| Question | Answer |
| --- | --- |
| Collects data? | Yes — limited technical data for presence and temporary delivery |
| Account / phone / email | Not required. No cloud account |
| Messages | Not collected as plaintext. Text is end-to-end sealed. The delivery server stores opaque ciphertext until ack or expiry |
| Contacts | Stored on device only |
| Audio / video | Call media is end-to-end encrypted. The coordination bridge does not decrypt it. Not stored on our servers |
| Location | No location permission. Optional IP hints are connection endpoints, not a location product |
| Encrypted in transit | Yes |
| Data sold | No |
| Ads / analytics in the app | No advertising SDK and no third-party analytics in the app. ghalbol.com may use Firebase Analytics for page views only |
| Deletion | Uninstall or clear app storage. Presence expires. See https://ghalbol.com/data-deletion |
| Backups | `android:allowBackup="false"`. User-exported keystore backups are the user’s file |
| Children | Not directed at children under 13 |

Device or other IDs: the identity is a local key pair. The public key is registered for presence. We do not use the Android advertising ID.
