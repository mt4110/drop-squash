export function formatBytes(bytes: number) {
  if (bytes < 1_000) {
    return `${bytes} B`;
  }

  if (bytes < 1_000_000) {
    return new Intl.NumberFormat(undefined, {
      maximumFractionDigits: 1,
      style: "unit",
      unit: "kilobyte",
    }).format(bytes / 1_000);
  }

  return new Intl.NumberFormat(undefined, {
    maximumFractionDigits: 1,
    style: "unit",
    unit: "megabyte",
  }).format(bytes / 1_000_000);
}

export function fileName(path: string) {
  const segments = path.split(/[\\/]/);
  return segments[segments.length - 1] || path;
}

export function parentPath(path: string) {
  const separator = Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\"));
  return separator > 0 ? path.slice(0, separator) : path;
}

export function displayPath(path: string) {
  return path
    .replace(/^\/Users\/[^/]+/, "~")
    .replace(/^\/home\/[^/]+/, "~");
}
