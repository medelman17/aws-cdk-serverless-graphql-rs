#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { ServiceStack } from "../lib/service-stack";
import * as path from "path";

const app = new cdk.App();
new ServiceStack(app, "ServiceStack", {
  handler_path:
    "/Users/michael/Documents/Code/github.com/medelman17/cdk-serverless-gql-rs/assets/graphql-handler-lambda/lambda.zip",
});
