import { McpError, ErrorCode } from '@modelcontextprotocol/sdk/types.js';
import {
  WaccInputSchema,
  CreditMetricsInputSchema,
  DcfInputSchema,
  DebtCapacityInputSchema,
  CovenantInputSchema,
} from './schemas.js';

// Import the native bindings (napi-rs generates camelCase names)
import {
  waccCalculator,
  creditMetrics,
  dcfModel,
  debtCapacity,
  covenantCompliance,
} from '@corp-finance/bindings';

/**
 * Tool handler for WACC calculator
 */
export async function handleWaccCalculator(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    // Validate input with Zod
    const validatedInput = WaccInputSchema.parse(args);

    // Convert to JSON string for Rust binding
    const inputJson = JSON.stringify(validatedInput);

    // Call Rust function via napi binding
    const resultJson = waccCalculator(inputJson);

    // Parse result
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
        `WACC calculation failed: ${error.message}`
      );
    }
    throw error;
  }
}

/**
 * Tool handler for credit metrics
 */
export async function handleCreditMetrics(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    const validatedInput = CreditMetricsInputSchema.parse(args);
    const inputJson = JSON.stringify(validatedInput);
    const resultJson = creditMetrics(inputJson);
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
        `Credit metrics calculation failed: ${error.message}`
      );
    }
    throw error;
  }
}

/**
 * Tool handler for DCF model
 */
export async function handleDcfModel(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    const validatedInput = DcfInputSchema.parse(args);
    const inputJson = JSON.stringify(validatedInput);
    const resultJson = dcfModel(inputJson);
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
        `DCF model calculation failed: ${error.message}`
      );
    }
    throw error;
  }
}

/**
 * Tool handler for debt capacity
 */
export async function handleDebtCapacity(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    const validatedInput = DebtCapacityInputSchema.parse(args);
    const inputJson = JSON.stringify(validatedInput);
    const resultJson = debtCapacity(inputJson);
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
        `Debt capacity calculation failed: ${error.message}`
      );
    }
    throw error;
  }
}

/**
 * Tool handler for covenant compliance
 */
export async function handleCovenantCompliance(args: unknown): Promise<{ content: Array<{ type: string; text: string }> }> {
  try {
    const validatedInput = CovenantInputSchema.parse(args);
    const inputJson = JSON.stringify(validatedInput);
    const resultJson = covenantCompliance(inputJson);
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
        `Covenant compliance check failed: ${error.message}`
      );
    }
    throw error;
  }
}
