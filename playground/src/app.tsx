import init, { supported_cameras } from "libraw-wasm";
import wasm from "libraw-wasm/libraw_wasm_bg.wasm?url";
import { useEffect, useState } from "preact/hooks";

const initializePromise = init({ module_or_path: wasm });
const dedupe = <T,>(arr: T[]): T[] => Array.from(new Set(arr));

export function App() {
	const [cameras, setCameras] = useState<string[]>();

	useEffect(() => {
		initializePromise.then(() => {
			setCameras((p) => p || dedupe(supported_cameras()));
		});
	});

	return (
		<>
			<h1>Vite + Preact</h1>
			{cameras ? (
				<ul>
					{cameras.map((camera) => (
						<li key={camera}>{camera}</li>
					))}
				</ul>
			) : (
				<p>Loading...</p>
			)}
		</>
	);
}
