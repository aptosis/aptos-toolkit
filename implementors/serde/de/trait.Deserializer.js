(function() {var implementors = {};
implementors["bincode"] = [{"text":"impl&lt;'de, 'a, R, O&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a mut <a class=\"struct\" href=\"bincode/de/struct.Deserializer.html\" title=\"struct bincode::de::Deserializer\">Deserializer</a>&lt;R, O&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"bincode/de/read/trait.BincodeRead.html\" title=\"trait bincode::de::read::BincodeRead\">BincodeRead</a>&lt;'de&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;O: <a class=\"trait\" href=\"bincode/config/trait.Options.html\" title=\"trait bincode::config::Options\">Options</a>,&nbsp;</span>","synthetic":false,"types":["bincode::de::Deserializer"]}];
implementors["config"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"config/struct.Value.html\" title=\"struct config::Value\">Value</a>","synthetic":false,"types":["config::value::Value"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"config/struct.Config.html\" title=\"struct config::Config\">Config</a>","synthetic":false,"types":["config::config::Config"]}];
implementors["erased_serde"] = [{"text":"impl&lt;'de, 'a&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a mut dyn <a class=\"trait\" href=\"erased_serde/trait.Deserializer.html\" title=\"trait erased_serde::Deserializer\">Deserializer</a>&lt;'de&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'de, 'a&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a mut (dyn <a class=\"trait\" href=\"erased_serde/trait.Deserializer.html\" title=\"trait erased_serde::Deserializer\">Deserializer</a>&lt;'de&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>)","synthetic":false,"types":[]},{"text":"impl&lt;'de, 'a&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a mut (dyn <a class=\"trait\" href=\"erased_serde/trait.Deserializer.html\" title=\"trait erased_serde::Deserializer\">Deserializer</a>&lt;'de&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>)","synthetic":false,"types":[]},{"text":"impl&lt;'de, 'a&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a mut (dyn <a class=\"trait\" href=\"erased_serde/trait.Deserializer.html\" title=\"trait erased_serde::Deserializer\">Deserializer</a>&lt;'de&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>)","synthetic":false,"types":[]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.61.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"erased_serde/trait.Deserializer.html\" title=\"trait erased_serde::Deserializer\">Deserializer</a>&lt;'de&gt;&gt;","synthetic":false,"types":["alloc::boxed::Box"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.61.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"erased_serde/trait.Deserializer.html\" title=\"trait erased_serde::Deserializer\">Deserializer</a>&lt;'de&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>&gt;","synthetic":false,"types":["alloc::boxed::Box"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.61.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"erased_serde/trait.Deserializer.html\" title=\"trait erased_serde::Deserializer\">Deserializer</a>&lt;'de&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;","synthetic":false,"types":["alloc::boxed::Box"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.61.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"erased_serde/trait.Deserializer.html\" title=\"trait erased_serde::Deserializer\">Deserializer</a>&lt;'de&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;","synthetic":false,"types":["alloc::boxed::Box"]}];
implementors["serde_hjson"] = [{"text":"impl&lt;Iter&gt; <a class=\"trait\" href=\"https://docs.serde.rs/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a> for <a class=\"struct\" href=\"serde_hjson/de/struct.Deserializer.html\" title=\"struct serde_hjson::de::Deserializer\">Deserializer</a>&lt;Iter&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Iter: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.61.0/std/primitive.u8.html\">u8</a>&gt;,&nbsp;</span>","synthetic":false,"types":["serde_hjson::de::Deserializer"]},{"text":"impl <a class=\"trait\" href=\"https://docs.serde.rs/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a> for <a class=\"struct\" href=\"serde_hjson/value/struct.Deserializer.html\" title=\"struct serde_hjson::value::Deserializer\">Deserializer</a>","synthetic":false,"types":["serde_hjson::value::Deserializer"]}];
implementors["serde_json"] = [{"text":"impl&lt;'de, 'a, R:&nbsp;<a class=\"trait\" href=\"serde_json/de/trait.Read.html\" title=\"trait serde_json::de::Read\">Read</a>&lt;'de&gt;&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a mut <a class=\"struct\" href=\"serde_json/struct.Deserializer.html\" title=\"struct serde_json::Deserializer\">Deserializer</a>&lt;R&gt;","synthetic":false,"types":["serde_json::de::Deserializer"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"enum\" href=\"serde_json/enum.Value.html\" title=\"enum serde_json::Value\">Value</a>","synthetic":false,"types":["serde_json::value::Value"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'de <a class=\"enum\" href=\"serde_json/enum.Value.html\" title=\"enum serde_json::Value\">Value</a>","synthetic":false,"types":["serde_json::value::Value"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"serde_json/value/struct.Number.html\" title=\"struct serde_json::value::Number\">Number</a>","synthetic":false,"types":["serde_json::number::Number"]},{"text":"impl&lt;'de, 'a&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a <a class=\"struct\" href=\"serde_json/value/struct.Number.html\" title=\"struct serde_json::value::Number\">Number</a>","synthetic":false,"types":["serde_json::number::Number"]}];
implementors["serde_urlencoded"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"serde_urlencoded/struct.Deserializer.html\" title=\"struct serde_urlencoded::Deserializer\">Deserializer</a>&lt;'de&gt;","synthetic":false,"types":["serde_urlencoded::de::Deserializer"]}];
implementors["serde_value"] = [{"text":"impl&lt;'de, E&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"serde_value/struct.ValueDeserializer.html\" title=\"struct serde_value::ValueDeserializer\">ValueDeserializer</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"serde/de/trait.Error.html\" title=\"trait serde::de::Error\">Error</a>,&nbsp;</span>","synthetic":false,"types":["serde_value::de::ValueDeserializer"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"enum\" href=\"serde_value/enum.Value.html\" title=\"enum serde_value::Value\">Value</a>","synthetic":false,"types":["serde_value::Value"]}];
implementors["serde_yaml"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"serde_yaml/struct.Deserializer.html\" title=\"struct serde_yaml::Deserializer\">Deserializer</a>&lt;'de&gt;","synthetic":false,"types":["serde_yaml::de::Deserializer"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"serde_yaml/struct.Number.html\" title=\"struct serde_yaml::Number\">Number</a>","synthetic":false,"types":["serde_yaml::number::Number"]},{"text":"impl&lt;'de, 'a&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a <a class=\"struct\" href=\"serde_yaml/struct.Number.html\" title=\"struct serde_yaml::Number\">Number</a>","synthetic":false,"types":["serde_yaml::number::Number"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"enum\" href=\"serde_yaml/enum.Value.html\" title=\"enum serde_yaml::Value\">Value</a>","synthetic":false,"types":["serde_yaml::value::Value"]}];
implementors["toml"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"enum\" href=\"toml/value/enum.Value.html\" title=\"enum toml::value::Value\">Value</a>","synthetic":false,"types":["toml::value::Value"]},{"text":"impl&lt;'de, 'b&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'b mut <a class=\"struct\" href=\"toml/de/struct.Deserializer.html\" title=\"struct toml::de::Deserializer\">Deserializer</a>&lt;'de&gt;","synthetic":false,"types":["toml::de::Deserializer"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()