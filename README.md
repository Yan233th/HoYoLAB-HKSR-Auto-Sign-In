# HoYoLAB-HKSR-Auto-Sign-In
Auto-Sign-In for Honkai: Star Rail on HoYoLAB

## Usage Instructions
Fork the repository and run the `Build and Release (Linux-x64 Only)` Workflow. Afterward, enable the `Scheduled Auto Sign In` Workflow. (Note: Before running this workflow, make sure there is a release containing properly compiled Linux binaries.) Then, store the tokens in `USER_TOKENS` under secrets.
> Repository Settings -> Secrets and variables -> Actions -> Repository secrets

Please manually retrieve the cookies from the HoYoLAB website and store them in the `USER_TOKENS` environment variable, separated by a `|` for different users. Each user must have their respective `ltoken_v2` and `ltuid_v2` for proper login validation.