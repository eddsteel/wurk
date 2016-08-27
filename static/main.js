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
                _.map(result.photos, console.log);
                push_frame(result.photos);
                $(img(result.photos[0], count)).appendTo(target);
            }
        });
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
