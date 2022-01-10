// Reference: https://github.com/Marvinified/use-file-upload
export const onChangeFile = (e: any, callback: (r: Record<string, any>) => void) => {
  const parsedFiles = []
  const target = e.target

  // Loop through files
  for (const fileIndex in target.files) {
    // check if index is a number
    if (isNaN(fileIndex as unknown as number)) {
      continue
    }

    // get file object
    const file = target.files[fileIndex]

    // select properties

    const parsedFile = {
      source: URL.createObjectURL(file),
      name: file.name,
      size: file.size,
      file, // original file object
    }

    // add to parsed file list
    parsedFiles.push(parsedFile)
  }

  // remove event listener after operation
  target.removeEventListener('change', onChangeFile)

  // remove input element after operation
  target.remove()

  // If multiple select
  if (target.multiple) {
    return callback(parsedFiles)
  }

  return callback(parsedFiles[0])
}
