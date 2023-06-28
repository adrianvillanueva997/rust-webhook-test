pub async fn update_vxtwitter(message: &str) -> String {
    //TODO: Optmize this function
    if message.contains("fxtwitter") || message.contains("vxtwitter") {
        return String::from("");
    }
    if message.contains("twitter") && message.contains("status") {
        let url = message.replace("twitter", "vxtwitter");

        return url;
    }
    return String::from("");
}
