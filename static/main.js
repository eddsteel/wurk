function Wurk() {
    var counter = 0;
    var frames = [];

    function push_frame(frame) {
        frames.push(frame)
    }

    function img(photo, count) {
        return "<img id=\"burst-" + count + "\" src=\"" + photo + "\"/>\n";
    }

    function inject_photos(target, count) {
        jQuery.ajax({
            url: "/photos",
            dataType: 'json',
            success: function(result) {
                push_frame(result.photos);
                var photo = $(img(result.photos[0], count));
                photo.appendTo(target);
                animate_step(photo, result.photos, count, Date.now());
            }
        });
    }


    function animate_step(img, photos, count, then) {
        var now = Date.now();

        if ((now - then) > 75) {
            img.attr("src", photos[count]);
            var new_count;
            if (count >= photos.length) {
                new_count = 0;
            } else {
                new_count = count + 1;
            }
            animate_step(img, photos, new_count, now);
        } else {
            requestAnimationFrame(function() { animate_step(img, photos, count, then) });
        }
    }

    function animate(counter) {
        var img = $("#burst-" + counter);
        var photos = frames[counter];
        animate_step(img, photos, 0);
    }


    this.go = function() {
        inject_photos("#frame", counter, frames);
        counter++;
    }

    this.f = function() {
        return frames;
    }

    this.inject = function(button) {
        button.click(this.go)
    }
};
