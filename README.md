RDR2 screenshots are stored in a weird format. This tool takes a path to your RDR2 profile folder,
converts all screenshots to JPGs, and puts them in `screenshots/`. The original versions are put in
`screenshots_backup/` so you can recover them and upload them to the social club if you wish.

### Details
On Windows, the RDR2 profile folder is located at
`C:\Users\{USERNAME}\Documents\Rockstar Games\Red Dead Redemption 2\Profiles\{random letters here}`.
Each screenshot's filename starts with `PRDR`, and they're just JPGs with 300 extra bytes at the
start.
