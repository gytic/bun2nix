# Set of Bun packages to install
{
  "@workspace/app" = {
    out_path = "@workspace/app";
    name = "@workspace/app@workspace:packages/app";
    url = "https://registry.npmjs.org/@workspace/app/-/app-workspace:packages/app.tgz";
    hash = "sha512-workspaceDummyHash";
  };
  "@workspace/lib" = {
    out_path = "@workspace/lib";
    name = "@workspace/lib@workspace:packages/lib";
    url = "https://registry.npmjs.org/@workspace/lib/-/lib-workspace:packages/lib.tgz";
    hash = "sha512-workspaceDummyHash";
  };
  "chalk" = {
    out_path = "chalk";
    name = "chalk@5.4.1";
    url = "https://registry.npmjs.org/chalk/-/chalk-5.4.1.tgz";
    hash = "sha512-zgVZuo2WcZgfUEmsn6eO3kINexW8RAE4maiQ8QNs8CtpPCSyMiYsULR3HQYkm3w8FIA3SberyMJMSldGsW+U3w==";
  };
  "is-number" = {
    out_path = "is-number";
    name = "is-number@6.0.0";
    url = "https://registry.npmjs.org/is-number/-/is-number-6.0.0.tgz";
    hash = "sha512-Wu1VHeILBK8KAWJUAiSZQX94GmOE45Rg6/538fKwiloUu21KncEkYGPqob2oSZ5mUT73vLGrHQjKw3KMPwfDzg==";
  };
  "is-odd" = {
    out_path = "is-odd";
    name = "is-odd@3.0.1";
    url = "https://registry.npmjs.org/is-odd/-/is-odd-3.0.1.tgz";
    hash = "sha512-CQpnWPrDwmP1+SMHXZhtLtJv90yiyVfluGsX5iNCVkrhQtU3TQHsUWPG9wkdk9Lgd5yNpAg9jQEo90CBaXgWMA==";
  };
}