# How to use
## 1. Install

```bash
cargo install envault
```

## 2. Describe your environment variables in yaml

```yaml
staging:
  AWS_ACCESS_KEY_ID: xxx
  AWS_SECRET_ACCESS_KEY: yyy
  AWS_DEFAULT_REGION: zzz
```

## 3. Set environment variables


```shell
export ENV_KEY__staging__AWS_ACCESS_KEY_ID=password
export ENV_KEY__staging=password
export ENV_KEY=password
```


## 4. Encrypt yaml

```shell
envault encrypt --raw .env.yaml [--enc .env.enc.yaml [--env staging]]
```


## 5. Load environment variables to export

```shell
envault export --enc .env.enc.yaml --env staging
```
