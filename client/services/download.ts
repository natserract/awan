import { serializeFileUrl } from '../utils/string'

export async function downloadFile(fileUrl: string) {
  const file = await fetch(fileUrl)
  const fileBlob = await file.blob()

  const nameSplit = fileUrl.split('/')
  const popName = nameSplit.pop()
  const fileName = serializeFileUrl(popName!!)

  // Convert your blob into a Blob URL (a special url that points to an object in the browser's memory)
  const blobUrl = URL.createObjectURL(fileBlob)

  // Create a link element
  const link = document.createElement('a')

  // Set link's href to point to the Blob URL
  link.href = blobUrl
  link.download = fileName

  // Append link to the body
  document.body.appendChild(link)

  // Dispatch click event on the link
  // This is necessary as link.click() does not work on the latest firefox
  link.dispatchEvent(
    new MouseEvent('click', {
      bubbles: true,
      cancelable: true,
      view: window,
    })
  )

  // Remove link from body
  document.body.removeChild(link)

  return
}
