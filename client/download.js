async function downloadFile(sid, info, lyrics) {
    // Make a POST request to download song
    let resp = await fetch(`http://127.0.0.1:3000/api/download/${sid}`, {
        method: "POST",
        mode: "cors",
        body: JSON.stringify({ info, lyrics }),
        headers: {
            "Content-Type": "application/json",
        }
    });

    // Get filename from HTTP Header
    const filename = resp.headers.get('content-disposition').split("filename")[1].slice(2, -1) + ".mp3"

    // Create a Download link for File
    let blob = await resp.blob();
    const link = window.URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = link;
    a.download = filename;

    // Firefox requires the link to be added to the DOM before click()
    a.onclick = e => {
        document.body.removeChild(e.target); // destroyClickedElement
        e.stopPropagation();
    };
    a.style.display = "none";
    document.body.appendChild(a);

    // Safari will open this file in the same page as a confusing Blob.
    if (this._isSafari) {
        let aText = 'Hello, Safari user! To download this file...\n';
        aText += '1. Go to File --> Save As.\n';
        aText += '2. Choose "Page Source" as the Format.\n';
        aText += `3. Name it the file: ${filename}`;
        alert(aText);
    }
    a.click();
}