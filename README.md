# p2p-exchange

This repository is yet another demonstration using WebRTC 
for peer-to-peer message exchange.

Live version: <https://webrtc.charentenay.me>

# Running Locally
The following are required to run locally:
* rust
* npm / nodejs

## Signaling Server

# Deploying to AWS
The following are required to compile locally and deploy to AWS:
* rust
* npm / nodejs
* terraform

## AWS Infrastructure using Terraform

The AWS infrastructure can be managed using Terraform (<https://www.terraform.io/>).
The terraform script corresponding to this project is located in the
directory `terraform`. The repository contains two templates files `vars/template.tfvars` and `vars/template.hcl` that respectively correpond to a set of variables used in the terraform scripts and parameters defining the terraform backend - this terraform script uses Terraform Cloud.

These files need to be tailored to your situation. Assuming that you are also using terraform cloud, the recommended approach is as follows:
```
cd terraform/vars
cp template.tfvars prod.tfvars
# and replace ${environment}, ${domain} and ${subdomains} variables in prod.tfvars

cp template.hcl prod.hcl
# and replace ${organisation} and ${workspace_name} in prod.hcl
```

Run the following terraform commands from the `terraform` directory:
```
terraform init --var-file=./vars/prod.tfvars --backend-config=./vars/prod.hcl
terraform plan --var-file=./vars/prod.tfvars --out=terraform_plan
terraform apply terraform_plan
```

## `www`

```
ENVIRONMENT=prod
npm run build
aws s3 rm --recursive s3://webrtc-www-$ENVIRONMENT/
aws s3 cp --recursive dist s3://webrtc-www-$ENVIRONMENT/
cloudfrontId=`aws ssm get-parameter --name "/webrtc/$ENVIRONMENT/cloudfront/id" | jq -r '.Parameter.Value'`
aws cloudfront create-invalidation --distribution-id $cloudfrontId --paths "/*"
```


## `lambda`

```
cargo build --release --bin lambda --target x86_64-unknown-linux-musl
ldd target/x86_64-unknown-linux-musl/release/lambda
cp target/x86_64-unknown-linux-musl/release/lambda ./bootstrap
zip bootstrap.zip ./bootstrap
aws lambda update-function-code --function-name webrtc-prod --zip-file fileb://./bootstrap.zip
rm -f bootstrap.zip ./bootstrap
```
