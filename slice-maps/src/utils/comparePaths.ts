import path from 'node:path'
export function isChildOf(pathToCheck: string, basePath: string,): boolean {
  const relative = path.relative(basePath, pathToCheck)

  return relative !== '' &&
    relative !== '..' &&
    !relative.startsWith(`..${path.sep}`) &&
    !path.isAbsolute(relative)
}

export function hasBasePath(
  paths: Iterable<string>,
  path: string
): boolean {
  for (const basePath of paths) {
    if (isChildOf(path, basePath)) {
      return true
    }
  }

  return false
}