import preact from "@preact/preset-vite";
import { type PluginOption, defineConfig } from "vite";

const wasmEnv = () => {
	const resolved = "\0env";
	const envImports = [
		"strerror",
		"_Znwm",
		"_ZdlPvm",
		"__cxa_allocate_exception",
		"__cxa_throw",
		"sscanf",
		"ntohs",
		"qsort",
		"_ZNSt12length_errorD1Ev",
		"_ZNSt11logic_errorC2EPKc",
		"_ZNSt20bad_array_new_lengthD1Ev",
		"_ZNSt20bad_array_new_lengthC1Ev",
		"free",
		"calloc",
		"malloc",
		"strcasecmp",
		"htonl",
		"strstr",
		"strcpy",
		"atof",
		"strncat",
		"siprintf",
		"strncasecmp",
		"strcmp",
		"htons",
		"snprintf",
		"strchr",
		"isspace",
		"strncmp",
		"mktime",
		"atoi",
		"strtok_r",
		"strncpy",
		"isalnum",
		"strcat",
		"strrchr",
		"fiprintf",
		"fwrite",
		"memchr",
		"strtol",
		"swab",
		"localtime",
		"ntohl",
		"__small_fprintf",
	];
	return {
		name: "wasm-env",
		resolveId(id) {
			if (id === "env") return resolved;
		},
		load(id) {
			if (id === resolved)
				return envImports.map((n) => `export const ${n} = () => {};`).join("");
		},
	} satisfies PluginOption;
};
// https://vite.dev/config/
export default defineConfig({
	plugins: [preact(), wasmEnv()],
});
