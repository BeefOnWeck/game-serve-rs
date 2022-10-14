Generate a `fly.toml` file by running `fly launch`.

```bash
fly auth login
fly launch
```

This will fail the first time because no command is defined to run the binary.

Modify the experimental section to add the command to run.

```toml
[experimental]
  allowed_public_ports = []
  auto_rollback = true
  cmd = "./game-serve-rs"
```

Then run `fly launch` again and this time it should create a healthy instance.