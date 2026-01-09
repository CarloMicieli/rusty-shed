# Security Policy

Thank you for helping keep this project secure. This document explains how to report security vulnerabilities privately and how we handle reports.

## Reporting a vulnerability

Preferred channels (in order):

1. GitHub Security Advisories (recommended): open a private security advisory for this repository.
2. Email: send a report to `security@rusty-shed.io`. If possible, encrypt the message with the PGP key below.

When reporting, please include:

- Repository name and affected package(s) and version(s)
- A clear description of the vulnerability and its impact
- Steps to reproduce (minimal proof-of-concept if possible)
- Any logs, stack traces, or screenshots that help reproduce the issue
- Your preferred disclosure timeline and contact information

Do NOT post vulnerability details publicly (issues, PRs, or public discussion) until a coordinated disclosure has occurred.

## Response and timelines

- Acknowledgement: We aim to acknowledge valid reports within 3 business days.
- Triage: We will assess severity and provide an initial triage estimate within 7 business days.
- Fix & Disclosure: We strive to release a fix and coordinated disclosure within 90 days for non-critical vulnerabilities; critical/zero-day vulnerabilities may follow an accelerated timeline.
- CVE assignment: We will request a CVE for issues that meet criteria and coordinate attribution and disclosure details.

If you need a faster response due to an active exploit or data loss risk, include `URGENT` in the subject and use the PGP-encrypted email channel.

## Supported releases

We primarily support the `main` branch and the most recent release branches. If the vulnerability affects older releases, we will evaluate backporting fixes based on severity and maintenance resources.

## Maintainer workflow (how we handle reports)

1. Acknowledge the reporter and request additional info if needed.
2. Triage severity and scope impact.
3. Prepare a fix on a private branch and test it.
4. Coordinate a private disclosure (release notes and advisory) and publish a security advisory.
5. Merge and release fixes to public branches and create any necessary backports.

## Public disclosure

We will publish a security advisory on GitHub and a short disclosure note in the repository's release notes once a fix is available and coordinated with the reporter.

## Contact

- Security contact: `security@rusty-shed.io`
- For non-security issues, use regular issues or feature requests.

_Last updated: 2026-01-09_
