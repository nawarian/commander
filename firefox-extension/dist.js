browser.contextMenus.create(
  {
    id: "open-video",
    title: "Open video",
    contexts: ["link"],
  },
  () => void browser.runtime.lastError,
);

async function handleOpenMpvVideo(info, tab) {
  const youtubeUrl = new URL(info.linkUrl)

  const url = new URL(`http://localhost:7171/youtube?format=mp4&video=${youtubeUrl.searchParams.get('v')}`)
  await fetch(url, {
    method: "POST",
    headers: {
      "content-type": "application/json",
    }
  });
}

browser.contextMenus.onClicked.addListener(async (info, tab) => {
  switch (info.menuItemId) {
    case "open-video":
      await handleOpenMpvVideo(info, tab);
      break;
    default:
      break;
  }
});
