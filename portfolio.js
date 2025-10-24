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

	// Nothing to do
	if(!about || !bg) return;

	aboutToggle.addEventListener('click', (event) => {
		event.preventDefault();

		about.classList.toggle('active');
		bg.classList.toggle('active');
	});
});