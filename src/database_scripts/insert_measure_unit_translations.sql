INSERT INTO general.measure_unit_translations (measure_unit_id, langauge_code, name) VALUES

((SELECT id FROM general.measure_units WHERE abbreviation = 'Ml'), 'en-US', 'Milliliter'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Ml'), 'de-DE', 'Milliliter'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Cl'), 'en-US', 'Centiliter'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Cl'), 'de-DE', 'Zentiliter'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'L'), 'en-US', 'Liter'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'L'), 'de-DE', 'Liter'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Oz'), 'en-US', 'Fluid Ounce'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Oz'), 'de-DE', 'Unze'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Drop'), 'en-US', 'Drop'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Drop'), 'de-DE', 'Tropfen'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Tsp'), 'en-US', 'Teaspoon'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Tsp'), 'de-DE', 'Teelöffel'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Tbsp'), 'en-US', 'Tablespoon'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Tbsp'), 'de-DE', 'Esslöffel'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Dash'), 'en-US', 'Dash'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Dash'), 'de-DE', 'Schuss'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Splash'), 'en-US', 'Splash'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Splash'), 'de-DE', 'Spritzer'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'G'), 'en-US', 'Gram'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'G'), 'de-DE', 'Gramm'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Kg'), 'en-US', 'Kilogram'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Kg'), 'de-DE', 'Kilogramm'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Mg'), 'en-US', 'Milligram'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Mg'), 'de-DE', 'Milligramm'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Lb'), 'en-US', 'Pound'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Lb'), 'de-DE', 'Pfund'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'OzWt'), 'en-US', 'Ounce (Weight)'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'OzWt'), 'de-DE', 'Unze (Gewicht)'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Ton'), 'en-US', 'Metric Ton'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Ton'), 'de-DE', 'Tonne (metrisch)'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Pcs'), 'en-US', 'Piece'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Pcs'), 'de-DE', 'Stück'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Leaf'), 'en-US', 'Leaf'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Leaf'), 'de-DE', 'Blatt'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Clove'), 'en-US', 'Clove'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Clove'), 'de-DE', 'Zehe'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Stick'), 'en-US', 'Stick'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Stick'), 'de-DE', 'Stange'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Pinch'), 'en-US', 'Pinch'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Pinch'), 'de-DE', 'Prise'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Ms'), 'en-US', 'Knife Tip'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Ms'), 'de-DE', 'Messerspitze'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Bunch'), 'en-US', 'Bunch'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Bunch'), 'de-DE', 'Bund')
ON CONFLICT (measure_unit_id, langauge_code) DO NOTHING;
