const currentTagList = new Set();

function thumbnailIsCompatibleWithTags(thumbnail) {
	// A thumbnail must have EVERY tag of the currentTagList to be compatible.
	for(const tag of currentTagList) {
		if(!thumbnail.classList.contains(tag)) {
			return false;
		}
	}

	return true;
}

function updateTags() {
	// Update thumbnails to hide non-tagged ones.
	document.querySelectorAll('.thumbnail').forEach(thumbnail => {
		if(thumbnailIsCompatibleWithTags(thumbnail)) {
			thumbnail.classList.remove('thumbnail-hidden');
		}
		else {
			thumbnail.classList.add('thumbnail-hidden');
		}
	});

	// Update tags to be bolded if they are current tags.
	document.querySelectorAll('.tag').forEach(tag => {
		// The class name for this tag.
		const tagName = `tag-${tag.innerHTML}`;

		if(currentTagList.has(tagName)) {
			tag.classList.add('tag-selected');
		}
		else {
			tag.classList.remove('tag-selected');
		}
	});
}

function toggleTag(tagName) {
	if(currentTagList.has(tagName)) {
		currentTagList.delete(tagName);
	}
	else {
		currentTagList.add(tagName);
	}
	updateTags();
}

document.querySelectorAll('.tag').forEach(tag => {
	// The class name for this tag.
	const tagName = `tag-${tag.innerHTML}`;

	// Add event listener for toggling the tag.
	tag.addEventListener('click', (event) => {
		event.preventDefault();
		toggleTag(tagName);
	});
});

document.querySelectorAll('.call.about').forEach(aboutToggle => {
	// Because this part of the DOM is static, the ideal thing is
	// to just capture all the nodes we care about upfront, so
	// that we don't have to perform the querySelector every time
	// the click is toggled.
	const thumbnail = aboutToggle.closest('.thumbnail');
	// Nothing to do
	if(!thumbnail) return;

	const about = thumbnail.querySelector('.thumbnail-about');
	const bg = thumbnail.querySelector('.thumbnail-about-bg');

	const video = thumbnail.querySelector('.thumbnail-video');
	if(video) {
		// Load videos when we first hover over them.
		thumbnail.addEventListener('mouseenter', (event) => {
			if(!video.src) {
				video.src = video.dataset.src;
				video.load();
			}
		});
	}

	// Nothing to do
	if(!about || !bg) return;

	aboutToggle.addEventListener('click', (event) => {
		event.preventDefault();

		about.classList.toggle('active');
		bg.classList.toggle('active');
	});
});