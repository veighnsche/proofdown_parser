# Mobile Testing Artifacts (iOS & Android)

Mobile test frameworks produce a mix of binary result bundles, JUnit XML, screenshots, and videos.

## iOS (XCTest/XCUITest)

- Runner: `xcodebuild test` (or via CI wrappers)
- Primary artifact: `.xcresult` bundle containing test logs, attachments, and screenshots
- Exports/conversions: tools can convert to JUnit XML and HTML dashboards
- Coverage: Xcode/LLVM coverage (exportable as profdata → HTML/JSON via llvm-cov)

Typical files:
- `Test.xcresult` bundle
- Derived JUnit XML: `TEST-*.xml`
- Screenshots: PNG files attached to steps

## Android (Instrumentation, Espresso, UI Automator)

- Runner: `AndroidJUnitRunner` (JUnit XML), Gradle test tasks
- Artifacts: `TEST-*.xml`, screenshots, logcat traces, videos (via device farm), coverage via JaCoCo

## Cross-platform/mobile end-to-end

- Appium (WebDriver-based): produces JUnit/NUnit/TestNG XML, screenshots, videos
- Detox (React Native): JUnit XML + artifacts
- Firebase Test Lab: device matrix results, videos, logs, JUnit XML

## Proofdown viewer mapping

- JUnit XML → convert to JSON summary → `artifact.json` and rollup `artifact.table`.
- `.xcresult` → package and link via `artifact.link`; extract a JSON summary of tests/attachments for `artifact.json`.
- Screenshots/videos → `artifact.image` and `artifact.link` for downloads.
- Coverage → treat as in the Coverage page (JaCoCo/LLVM/LCOV → JSON summary + links).

## Implications for Proofdown Language Spec

- `.xcresult` is a binary bundle; treat as opaque via `artifact.link` with a small `artifact.json` summary (counts, failures, attachments).
- Mobile runs often include many screenshots—use `grid`/`card` with `artifact.image` and encourage `max_height` to keep pages scannable.
- Normalize Android/iOS result formats to JSON; keep the grammar free of platform-specific constructs.
- Prefer digest-addressed artifacts to avoid path drift between device matrix runs.
- Keep renderer-side behavior (collapse, paging) implementation-specific; the language grammar stays minimal.

## References

- XCTest: https://developer.apple.com/documentation/xctest
- AndroidJUnitRunner: https://developer.android.com/training/testing/instrumented-tests/androidx-test-libraries/runner
- Appium JUnit reports: https://www.browserstack.com/docs/test-management/upload-reports-cli/frameworks/appium
