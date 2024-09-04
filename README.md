OpenTofu Provider proxy (in Rust)
=================================

The protocol between OpenTofu (or Terraform) and its providers is heavily reliant on Google's GRPC protocol, which is not inherently straightforward to implement.

This thing for now is a simple converter which runs and connects to an (already-downloaded) provider binary, fetches its schema and prints it out as JSON.
