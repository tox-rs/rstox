pub struct ToxAv {
	tox: Tox,
	av_rx: Receiver<Event>,
	av_tx: Sender<Event>,
}
