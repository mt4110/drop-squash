import type { ConversionSummary } from "../lib/commands";
import { fileName, formatBytes } from "../lib/format";

type ResultCardProps = {
  result: ConversionSummary;
};

export function ResultCard({ result }: ResultCardProps) {
  return (
    <section className="result" aria-label="Conversion result">
      <div>
        <span>Ready</span>
        <strong>{fileName(result.outputPath)}</strong>
      </div>
      <div>
        <span>Saved</span>
        <strong>{formatBytes(result.savedBytes)} ({result.reductionPercent.toFixed(1)}%)</strong>
      </div>
    </section>
  );
}
