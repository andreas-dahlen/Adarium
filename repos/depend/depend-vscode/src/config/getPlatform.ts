export function getPlatform(): string {
  return `${process.platform}-${process.arch}`;
}