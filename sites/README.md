# sites/

Public websites for the GhalBol brand, following the same layout as
Kumkaj (`kumkaj-workspace/sites/`) and Wadhu (`wadhu-workspace/sites/`):

```text
sites/<product>/
  client/           # Firebase JS sources (esbuild → static/js/)
  static/           # Source tree for Hosting (HTML, .well-known, downloads/)
  public/           # Staged Hosting root (gitignored; built by scripts)
  scripts/          # preview-web.sh, deploy-firebase.sh, …
  firebase.json
  .firebaserc
  package.json
```

| Site | Package | Domain |
| --- | --- | --- |
| [`ghalbol/`](ghalbol/README.md) | `ghalbol-website` | https://ghalbol.com |

Chat/calls live in `apps/ghalbol` + `ghalbol_core`, not in the browser.
The site is invite handoff, downloads, and privacy — plus Firebase Analytics.
