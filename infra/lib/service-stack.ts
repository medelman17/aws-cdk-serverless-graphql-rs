import { Stack, StackProps, CfnOutput } from "aws-cdk-lib";
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as apigateway from "@aws-cdk/aws-apigatewayv2-alpha";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as integrations from "@aws-cdk/aws-apigatewayv2-integrations-alpha";
import * as iam from "aws-cdk-lib/aws-iam";
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export interface ServiceStackProps extends StackProps {
  handler_path: string;
}

export class ServiceStack extends Stack {
  readonly database: dynamodb.Table;
  readonly gateway: apigateway.HttpApi;
  readonly gateway_integration: integrations.HttpLambdaIntegration;
  readonly lambda: lambda.Function;
  readonly lambda_role: iam.Role;

  constructor(scope: Construct, id: string, props: ServiceStackProps) {
    super(scope, id, props);

    this.database = new dynamodb.Table(this, "ServiceDatabase", {
      partitionKey: { name: "PK", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "SK", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      encryption: dynamodb.TableEncryption.AWS_MANAGED,
    });

    this.lambda_role = new iam.Role(this, "LambdaHandlerRole", {
      assumedBy: new iam.ServicePrincipal("lambda.amazonaws.com"),
    });

    this.lambda_role.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName(
        "service-role/AWSLambdaBasicExecutionRole"
      )
    );

    this.lambda = new lambda.Function(this, "LambdaHandlerFunction", {
      code: lambda.AssetCode.fromAsset(props.handler_path),
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: "not.required",
      tracing: lambda.Tracing.PASS_THROUGH,
      architecture: lambda.Architecture.ARM_64,
      role: this.lambda_role,
      insightsVersion: lambda.LambdaInsightsVersion.VERSION_1_0_119_0,
      environment: {
        TABLE_NAME: this.database.tableName,
      },
    });

    this.database.grantReadWriteData(this.lambda);

    this.gateway_integration = new integrations.HttpLambdaIntegration(
      "GatewayIntegration",
      this.lambda,
      {
        payloadFormatVersion: apigateway.PayloadFormatVersion.VERSION_2_0,
      }
    );

    this.gateway = new apigateway.HttpApi(this, "ServiceHttpApi", {
      createDefaultStage: true,
      defaultIntegration: this.gateway_integration,
    });

    new CfnOutput(this, "ApiEndpoint", {
      value: this.gateway.apiEndpoint,
    });

    new CfnOutput(this, "TableName", { value: this.database.tableName });
  }
}
