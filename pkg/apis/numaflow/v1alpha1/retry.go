package v1alpha1

import (
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
)

type OnFailRetryStrategy string

const (
	OnFailRetry    OnFailRetryStrategy = "retry"
	OnFailFallback OnFailRetryStrategy = "fallback"
	OnFailDrop     OnFailRetryStrategy = "drop"
)

type RetryStrategy struct {
	// +optional
	BackOff *Backoff `json:"backoff,omitempty" protobuf:"bytes,1,opt,name=backoff"`
	// +optional
	// +kubebuilder:default="retry"
	OnFailure OnFailRetryStrategy `json:"onFailure,omitempty" protobuf:"bytes,2,opt,name=onFailure"`
}

// Backoff holds parameters applied to a Backoff retry strategy.
type Backoff struct {
	// The initial duration.
	// +kubebuilder:default="1ms"
	// +optional
	Duration *metav1.Duration `json:"duration,omitempty" protobuf:"bytes,1,opt,name=duration"`
	// Duration is multiplied by factor each iteration, if factor is not zero
	// and the limits imposed by Steps and Cap have not been reached.
	// Should not be negative.
	// The jitter does not contribute to the updates to the duration parameter.
	// +optional
	// +kubebuilder:default=1
	Factor *int32 `json:"factor,omitempty" protobuf:"bytes,2,opt,name=factor"`
	// The sleep at each iteration is the duration plus an additional
	// amount chosen uniformly at random from the interval between
	// zero and `jitter*duration`.
	// +optional
	// +kubebuilder:default=0
	Jitter *int32 `json:"jitter,omitempty" protobuf:"bytes,3,opt,name=jitter"`
	// The remaining number of iterations in which the duration
	// parameter may change (but progress can be stopped earlier by
	// hitting the cap). If not positive, the duration is not
	// changed. Used for exponential backoff in combination with
	// Factor and Cap.
	// +optional
	// +kubebuilder:default=0
	Steps *int32 `json:"steps,omitempty" protobuf:"bytes,4,opt,name=steps"`
	// A limit on revised values of the duration parameter. If a
	// multiplication by the factor parameter would make the duration
	// exceed the cap then the duration is set to the cap and the
	// steps parameter is set to zero.
	// +kubebuilder:default="1ms"
	// +optional
	Cap *metav1.Duration `json:"cap,omitempty" protobuf:"bytes,5,opt,name=cap"`
}
