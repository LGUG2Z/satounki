package provider

import (
	"reflect"
)

func fieldDoc(kind interface{}, field string) string {
	t := reflect.TypeOf(kind)
	for i := 0; i < t.NumField(); i++ {
		tfsdk := t.Field(i).Tag.Get("tfsdk")
		if tfsdk == field {
			return t.Field(i).Tag.Get("rustdoc")
		}
	}

	return ""
}

func resourceDoc(kind interface{}) string {
	t := reflect.TypeOf(kind)
	return t.Field(0).Tag.Get("resourcedoc")
}
