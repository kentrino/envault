# What is this
"envault" /ɪnˈvɔːlt/ is a tool for encrypting confidential information such as API keys and managing them in a version control system.

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

You can use a single `ENV_KEY` environment variable to encrypt everything. You can also set environment variables for each environment and each confidential information individually.


```shell
export ENV_KEY__staging__AWS_ACCESS_KEY_ID=password
export ENV_KEY__staging=password
export ENV_KEY=password
```

## 4. Encrypt yaml

```shell
envault encrypt --raw .env.yaml [--enc .env.enc.yaml [--env staging]]
```

Then you will get the following YAML file.

```yaml
staging:
  AWS_SECRET_ACCESS_KEY: U2FsdGVkX19Qa0czd1huMUGSvIdM93bbWlEdzLCviak=
  AWS_DEFAULT_REGION: U2FsdGVkX19tMzdQQnBGRcOVStFTiP4P7/p8qt/T7TE=
  AWS_ACCESS_KEY_ID: U2FsdGVkX195RWNXdU52dpGBFIMYJ5kW8IoJPxkjioE=
```

Existing confidential information will be merged with new confidential information and outputted.

## 5. Load environment variables to export

```shell
envault export --enc .env.enc.yaml --env staging
```
