import { McpError, ErrorCode } from '@modelcontextprotocol/sdk/types.js';
import {
  ThreeStatementInputSchema,
  EquityEnterpriseBridgeInputSchema,
  DilutedSharesInputSchema,
  AccountingFlowInputSchema,
  FootballFieldInputSchema,
  PaperLboInputSchema,
} from './schemas.js';

// Import the native bindings (napi-rs generates camelCase names)
import {
  threeStatementModel,
  equityEnterpriseBridgeCalc,
  dilutedSharesCalc,
  accountingFlowAnalysis,
  footballFieldValuation,
  paperLboCalc,
} from '@corp-finance/bindings';

/**
 * Tool handler for three statement model
 */
export async function handleThreeStatementModel(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    const validatedInput = ThreeStatementInputSchema.parse(args);
    const inputJson = JSON.stringify(validatedInput);
    const resultJson = threeStatementModel(inputJson);
    const result = JSON.parse(resultJson);

    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(result, null, 2),
        },
      ],
    };
  } catch (error) {
    if (error instanceof Error) {
      throw new McpError(
        ErrorCode.InvalidRequest,
        `Three statement model calculation failed: ${error.message}`
      );
    }
    throw error;
  }
}

/**
 * Tool handler for equity/enterprise value bridge
 */
export async function handleEquityEnterpriseBridge(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    const validatedInput = EquityEnterpriseBridgeInputSchema.parse(args);
    const inputJson = JSON.stringify(validatedInput);
    const resultJson = equityEnterpriseBridgeCalc(inputJson);
    const result = JSON.parse(resultJson);

    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(result, null, 2),
        },
      ],
    };
  } catch (error) {
    if (error instanceof Error) {
      throw new McpError(
        ErrorCode.InvalidRequest,
        `Equity/Enterprise bridge calculation failed: ${error.message}`
      );
    }
    throw error;
  }
}

/**
 * Tool handler for diluted shares calculation
 */
export async function handleDilutedShares(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    const validatedInput = DilutedSharesInputSchema.parse(args);
    const inputJson = JSON.stringify(validatedInput);
    const resultJson = dilutedSharesCalc(inputJson);
    const result = JSON.parse(resultJson);

    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(result, null, 2),
        },
      ],
    };
  } catch (error) {
    if (error instanceof Error) {
      throw new McpError(
        ErrorCode.InvalidRequest,
        `Diluted shares calculation failed: ${error.message}`
      );
    }
    throw error;
  }
}

/**
 * Tool handler for accounting flow analysis
 */
export async function handleAccountingFlow(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    const validatedInput = AccountingFlowInputSchema.parse(args);
    const inputJson = JSON.stringify(validatedInput);
    const resultJson = accountingFlowAnalysis(inputJson);
    const result = JSON.parse(resultJson);

    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(result, null, 2),
        },
      ],
    };
  } catch (error) {
    if (error instanceof Error) {
      throw new McpError(
        ErrorCode.InvalidRequest,
        `Accounting flow analysis failed: ${error.message}`
      );
    }
    throw error;
  }
}

/**
 * Tool handler for football field valuation
 */
export async function handleFootballField(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    const validatedInput = FootballFieldInputSchema.parse(args);
    const inputJson = JSON.stringify(validatedInput);
    const resultJson = footballFieldValuation(inputJson);
    const result = JSON.parse(resultJson);

    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(result, null, 2),
        },
      ],
    };
  } catch (error) {
    if (error instanceof Error) {
      throw new McpError(
        ErrorCode.InvalidRequest,
        `Football field valuation failed: ${error.message}`
      );
    }
    throw error;
  }
}

/**
 * Tool handler for paper LBO
 */
export async function handlePaperLbo(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    const validatedInput = PaperLboInputSchema.parse(args);
    const inputJson = JSON.stringify(validatedInput);
    const resultJson = paperLboCalc(inputJson);
    const result = JSON.parse(resultJson);

    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(result, null, 2),
        },
      ],
    };
  } catch (error) {
    if (error instanceof Error) {
      throw new McpError(
        ErrorCode.InvalidRequest,
        `Paper LBO calculation failed: ${error.message}`
      );
    }
    throw error;
  }
}
